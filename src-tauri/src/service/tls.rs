use std::collections::HashMap;
use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MsbOption};
use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, PKeyRef, Private};
use openssl::rsa::Rsa;
use openssl::x509::extension::{
    AuthorityKeyIdentifier, BasicConstraints, KeyUsage, SubjectAlternativeName,
    SubjectKeyIdentifier,
};
use openssl::x509::{X509NameBuilder, X509Ref, X509Req, X509ReqBuilder, X509VerifyResult, X509};


pub fn mk_ca_cert_with_entry( entries :HashMap<String,String>,algorithm: String, bits_len: u32, not_before :u32,not_after :u32) -> Result<(X509, PKey<Private>), ErrorStack> {
    let rsa = Rsa::generate(bits_len)?;

    let mut key_pair = PKey::from_rsa(rsa)?;

    let mut x509_name = X509NameBuilder::new()?;

    for (key, value) in entries {
        x509_name.append_entry_by_text(key.as_str(),value.as_str())?;
    }

    let x509_name = x509_name.build();

    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(2)?;
    let serial_number = {
        let mut serial = BigNum::new()?;
        serial.rand(159, MsbOption::MAYBE_ZERO, false)?;
        serial.to_asn1_integer()?
    };
    cert_builder.set_serial_number(&serial_number)?;
    cert_builder.set_subject_name(&x509_name)?;
    cert_builder.set_issuer_name(&x509_name)?;
    cert_builder.set_pubkey(&key_pair)?;
    let not_before = Asn1Time::days_from_now(not_before)?;
    cert_builder.set_not_before(&not_before)?;
    let not_after = Asn1Time::days_from_now(not_after)?;
    cert_builder.set_not_after(&not_after)?;

    cert_builder.append_extension(BasicConstraints::new().critical().ca().build()?)?;
    cert_builder.append_extension(
        KeyUsage::new()
            .critical()
            .key_cert_sign()
            .crl_sign()
            .build()?,
    )?;

    let subject_key_identifier =
        SubjectKeyIdentifier::new().build(&cert_builder.x509v3_context(None, None))?;
    cert_builder.append_extension(subject_key_identifier)?;

    cert_builder.sign(&key_pair, MessageDigest::sha256())?;
    let cert = cert_builder.build();

    Ok((cert, key_pair))
}

pub fn mk_ca_cert_quick() -> Result<(X509, PKey<Private>), ErrorStack> {
    let mut entries :HashMap<String,String> = HashMap::new();

    entries.insert(String::from("C"), String::from("CN"));
    entries.insert(String::from("ST"), String::from("SC"));
    entries.insert(String::from("O"), String::from("META"));
    entries.insert(String::from("CN"), String::from("META QUICK CA"));

    mk_ca_cert_with_entry(entries,"rsa".to_string(),2048,0,3650)
}

/// Make a X509 request with the given private key
pub fn mk_request_with_entry(key_pair: &PKey<Private>,entries :HashMap<String,String>) -> Result<X509Req, ErrorStack> {
    let mut req_builder = X509ReqBuilder::new()?;
    req_builder.set_pubkey(key_pair)?;

    let mut x509_name = X509NameBuilder::new()?;
    for (key, value) in entries {
        x509_name.append_entry_by_text(key.as_str(),value.as_str())?;
    }

    let x509_name = x509_name.build();
    req_builder.set_subject_name(&x509_name)?;

    req_builder.sign(key_pair, MessageDigest::sha256())?;
    let req = req_builder.build();
    Ok(req)
}

pub fn mk_request_quick(key_pair: &PKey<Private>) -> Result<X509Req, ErrorStack> {
    let mut entries :HashMap<String,String> = HashMap::new();

    entries.insert(String::from("C"), String::from("cn"));
    entries.insert(String::from("ST"), String::from("sc"));
    entries.insert(String::from("O"), String::from("meta"));
    entries.insert(String::from("CN"), String::from("meta.quick.com"));

    mk_request_with_entry(key_pair,entries)
}

pub fn mk_reques(key_pair: &PKey<Private>,entries :HashMap<String,String>) -> Result<X509Req, ErrorStack> {
    mk_request_with_entry(key_pair,entries)
}

/// Make a certificate and private key signed by the given CA cert and private key
pub fn mk_ca_signed_cert(
    ca_cert: &X509Ref,
    ca_key_pair: &PKeyRef<Private>,
    dns_list: Vec<String>,
    entries: HashMap<String,String>,
    bits_len: u32,
    not_before: u32,
    not_after: u32
) -> Result<(X509, PKey<Private>), ErrorStack> {
    mk_ca_signed_cert_with_dns_with_enties(ca_cert,ca_key_pair,dns_list,entries,bits_len,not_before,not_after)
}


pub fn mk_ca_signed_cert_with_dns_with_enties(
    ca_cert: &X509Ref,
    ca_key_pair: &PKeyRef<Private>,
    dns_list :Vec<String>,
    entries :HashMap<String,String>,
    bits_len :u32,
    not_before: u32,
    not_after: u32,
) -> Result<(X509, PKey<Private>), ErrorStack> {
    let rsa = Rsa::generate(bits_len)?;
    let key_pair = PKey::from_rsa(rsa)?;

    let req = mk_request_with_entry(&key_pair,entries)?;

    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(2)?;
    let serial_number = {
        let mut serial = BigNum::new()?;
        serial.rand(159, MsbOption::MAYBE_ZERO, false)?;
        serial.to_asn1_integer()?
    };
    cert_builder.set_serial_number(&serial_number)?;
    cert_builder.set_subject_name(req.subject_name())?;
    cert_builder.set_issuer_name(ca_cert.subject_name())?;
    cert_builder.set_pubkey(&key_pair)?;
    let not_before = Asn1Time::days_from_now(not_before)?;
    cert_builder.set_not_before(&not_before)?;
    let not_after = Asn1Time::days_from_now(not_after)?;
    cert_builder.set_not_after(&not_after)?;

    cert_builder.append_extension(BasicConstraints::new().build()?)?;

    cert_builder.append_extension(
        KeyUsage::new()
            .critical()
            .non_repudiation()
            .digital_signature()
            .key_encipherment()
            .build()?,
    )?;

    let subject_key_identifier =
        SubjectKeyIdentifier::new().build(&cert_builder.x509v3_context(Some(ca_cert), None))?;
    cert_builder.append_extension(subject_key_identifier)?;

    let auth_key_identifier = AuthorityKeyIdentifier::new()
        .keyid(false)
        .issuer(false)
        .build(&cert_builder.x509v3_context(Some(ca_cert), None))?;
    cert_builder.append_extension(auth_key_identifier)?;

    let mut subject_alt_name = SubjectAlternativeName::new();
    for dns  in dns_list {
        subject_alt_name.dns(dns.as_str());
    }
    let x509_extension  = subject_alt_name.build(&cert_builder.x509v3_context(Some(ca_cert), None))?;
    cert_builder.append_extension( x509_extension )?;

    cert_builder.sign(ca_key_pair, MessageDigest::sha256())?;
    let cert = cert_builder.build();

    Ok((cert, key_pair))
}
fn real_main() -> Result<(), ErrorStack> {
    let (ca_cert, ca_key_pair) = mk_ca_cert_quick()?;
    let dns = vec!["datasafe-tech.com".to_string(),"meta.quick.org".to_string()];
    let mut entries = HashMap::<String,String>::new();
    entries.insert(String::from("C"), String::from("cn"));
    entries.insert(String::from("ST"), String::from("sc"));
    entries.insert(String::from("O"), String::from("meta"));
    entries.insert(String::from("CN"), String::from("meta.quick.com"));

    let (cert, _key_pair) = mk_ca_signed_cert(&ca_cert, &ca_key_pair,dns,entries,2048,0,3650)?;

    // Verify that this cert was issued by this ca
    match ca_cert.issued(&cert) {
        X509VerifyResult::OK => println!("Certificate verified!"),
        ver_err => println!("Failed to verify certificate: {}", ver_err),
    };

    Ok(())
}

pub fn load_ca(ca :String,pri_key :String) -> Result<(X509, PKey<Private>), ErrorStack> {
   let cert =  X509::from_pem(ca.as_bytes()).unwrap();
   let pri_ = PKey::private_key_from_pem(pri_key.as_bytes()).unwrap();

   Ok((cert,pri_))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_tls(){
        use openssl::ssl::{SslConnector, SslMethod};

        let mut ctx = SslConnector::builder(SslMethod::tls()).unwrap();
        println!("{:?}",ctx.as_ptr());
    }

    #[test]
    pub fn test_ca() ->  Result<(), ErrorStack>{
        let (ca_cert, ca_key_pair) = mk_ca_cert_quick()?;

        let dns = vec!["datasafe-tech.com".to_string(),"meta.quick.org".to_string()];

        let mut entries = HashMap::<String,String>::new();
        entries.insert(String::from("C"), String::from("cn"));
        entries.insert(String::from("ST"), String::from("sc"));
        entries.insert(String::from("O"), String::from("meta"));
        entries.insert(String::from("CN"), String::from("meta.quick.com"));

        let (cert, _key_pair) = mk_ca_signed_cert(&ca_cert, &ca_key_pair,dns,entries,2048,0,3650)?;

        match &ca_cert.to_pem() {
            Ok(result ) => {
                println!("{}",String::from_utf8_lossy(result));
            }
            _ => {}
        }

        match &ca_key_pair.public_key_to_pem() {
            Ok(result ) => {
                println!("{}",String::from_utf8_lossy(result));
            }
            _ => {}
        }

        match &ca_key_pair.private_key_to_pem_pkcs8() {
            Ok(result ) => {
                println!("{}",String::from_utf8_lossy(result));
            }
            _ => {}
        }

        // Verify that this cert was issued by this ca
        match ca_cert.issued(&cert) {
            X509VerifyResult::OK => println!("Certificate verified!"),
            ver_err => println!("Failed to verify certificate: {}", ver_err),
        };

        Ok(())
    }
}