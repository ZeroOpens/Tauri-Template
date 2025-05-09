<template>
  <div class="home">
    <div class="home-content">
      <p>tauri版本：{{ tauriVersion }}</p>
      <p>应用程序名称：{{ appName }}</p>
      <p>应用程序版本：{{ appVersion }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'Home'})
  import useTauri from '@/hooks/useTauri'


  const {tauriVersion, appName, appVersion} = useTauri()

</script>

<style scoped lang="scss">
.home {
  width: 100%;
  height: 100%;
  margin-top: 50px;

  p {
    text-align: center;
    font-size: 20px;
  }
}
</style>
 