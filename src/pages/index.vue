<template>
  <div class="index-container">
    <h1>Hello Tauri2.0</h1>
    <p>tauri版本：{{ tauriVersion }}</p>
    <p>应用程序名称：{{ appName }}</p>
    <p>应用程序版本：{{ appVersion }}</p>
    <Update/>
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'Index'})
  import useTauri from '@/hooks/useTauri'
  import Update from '@/components/update.vue'
  import { type userType, type usersType  } from '@/types'

  const {tauriVersion, appName, appVersion} = useTauri()

  // 用户类型声明
  let user:userType = { id: '01', userName: 'tauri', password: 231}
  console.log(user);

  let users:usersType = [
    { id: '01', userName: 'tauri', password: 231},
    { id: '02', userName: 'tauri2', password: 231},
    { id: '03', userName: 'tauri3', password: 231}
  ]
  console.log(users);

</script>

<style scoped>
.index-container {
  padding: 20px;
  max-width: 600px;
  margin: 0 auto;
  h1 {
    text-align: center;
  }
  p {
  font-size: 20px;
  }
}
</style>
 