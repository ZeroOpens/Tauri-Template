<template>
  <div class="index-container">
    <h1>Hello Tauri2.0</h1>
    <p>tauri版本：{{ tauriVersion }}</p>
    <p>应用程序名称：{{ appName }}</p>
    <p>应用程序版本：{{ appVersion }}</p>
    <Update/>
    <button @click="getDogImages">获取图片</button>
    <button @click="getWord">获取语句</button>
    <br>
    <p>{{ word }}</p>
    <img :src="dogImage">
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'Index'})
  import {ref} from 'vue'
  import useTauri from '@/hooks/useTauri'
  import Update from '@/components/update.vue'
  import { getDogAPI, getWordAPI } from '@/services/index'

  // 随机获取狗狗图片
  let dogImage = ref()
  const getDogImages = async () => {
    const response = await getDogAPI()
    dogImage.value = response.message
    console.log(response);
  };

  // 随机获取一句话
  let word = ref()
  const getWord = async () => {
    const response = await getWordAPI()
    word.value = response
    console.log(response);
  };


  const {tauriVersion, appName, appVersion} = useTauri()

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
  img{
    height: 200px;
  }
}
</style>
 