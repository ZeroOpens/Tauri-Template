<template>
  <div class="request">
    <div class="request-content">
      <button @click="getDogImages">获取图片</button>
      <button @click="getWord">获取语句</button>
      <br>
      <p>{{ word }}</p>
      <img :src="dogImage">
    </div>
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'Request'})
  import {ref} from 'vue'
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
</script>

<style scoped lang="scss">
.request {
  margin-top: 50px;
  display: flex;
  justify-content: center;

  button {
      margin: 0 30px;
      padding: 10px 20px;
      border-radius: 10px;
      background-color: #4CAF50;
      color: rgb(255, 255, 255);
      cursor: pointer;
      font-size: 16px;
      font-weight: bold;
  }
  p, img {
    width: 350px;
  }
}
</style>
 