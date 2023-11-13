// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod service;
mod cmd;
mod types;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmd::echo,
            cmd::savecluster,
            cmd::querycluster,
            cmd::queryselectedinstance,
            cmd::saveselectedinstance,
            cmd::etcd_all_key,
            cmd::etcd_delete_key,
            cmd::etcd_put_key,
            cmd::etcd_simpleput_key,
            cmd::list_roles,
            cmd::list_roles_permission,
            cmd::etcd_add_role,
            cmd::etcd_del_role,
            cmd::etcd_role_grant_perimssions,
            cmd::etcd_role_revoke_perimssions,
            cmd::etcd_user_list,
            cmd::etcd_user_role_list,
            cmd::etcd_user_add,
            cmd::etcd_user_delete,
            cmd::user_grant_role,
            cmd::user_revoke_role,
            cmd::pki_make_ca,
            cmd::pki_query_ca,
            cmd::mk_signed_cert,
            cmd::etcd_put_mapkey,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}