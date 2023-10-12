<template>
    <section class="app-main">
        <Suspense>
            <template #default>
                <router-view> </router-view>
            </template>
            <template #fallback> Loading... </template>
        </Suspense>
    </section>
</template>
  
<script lang="ts" setup name="app-name">
</script>
  
<style lang="scss" scoped>
.app-main {
    height: 100%;
    width: 100%;
    padding-top: 0px;
    position: relative;
    overflow: hidden;
}

.fixed-header+.app-main {
    padding-top: 5px;
}

.hasTagsView {
    .app-main {
        height: calc(100vh - 0px);
    }

    .fixed-header+.app-main {
        padding-top: 8px;
    }
}
</style>
  
<style lang="scss">
.gateway-popup-parent--hidden {
    .fixed-header {
        padding-right: 0px;
    }
}
</style>
  