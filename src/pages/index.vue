<template>
    <div class="index">
    <!-- 头部 -->
    <div class="index-header">
      <h1>Hello Tauri 2.0</h1>
    </div>
    <!-- logo -->
    <div class="hello" v-if="show">
      <img src="@/static/Images/logo.png" alt="">
      <button @click="setShow">进入学习</button>
    </div>
    <!-- 内容 -->
    <div class="content" v-else>
      <!-- 导航 -->
      <div class="index-nav">
        <RouterLink :to="{name: 'home'}" class="navLink" active-class="navLink-active">首页</RouterLink>
        <RouterLink :to="{name: 'pinia'}" class="navLink" active-class="navLink-active">pinia</RouterLink>
        <RouterLink :to="{name: 'request'}" class="navLink" active-class="navLink-active">请求</RouterLink>
        <RouterLink :to="{name: 'function'}" class="navLink" active-class="navLink-active">功能</RouterLink>
      </div>
      <!-- 展示区 -->
      <div class="index-content">
        <RouterView />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'Index'})
  import { RouterLink, RouterView } from 'vue-router'
  import { ref } from 'vue'

  let show = ref(true)

  const setShow = () => {
    show.value = !show
  }
</script>

<style scoped lang="scss">
.index {
  h1 {
    text-align: center;
  }

  .hello {
    img {
      display: block;
      width: 300px;
      margin: 0 auto;
    }
    button {
      display: block;
      margin: 0 auto;
      padding: 10px 20px;
      border-radius: 6px;
      background-color: #FFC131;
      color: rgb(0, 0, 0);
      border: none;
      cursor: pointer;
      font-size: 16px;
      font-weight: bold;
      transition: 0.2s linear; 
      // 悬浮状态样式
      &:hover {
        color: white;
        background-color: #24C8DB; 
        box-shadow: 10px -10px 10px #FFC131; 
      }
    }
  }

  // 导航样式
  .index-nav {
    display: flex;
    justify-content: center;
    gap: 12%;
    .navLink {
      text-decoration: none;
      padding: 8px 25px;
      border-radius: 15px;
      color: rgb(0, 0, 0);
      background: rgba(255, 255, 255);
      border: 3px solid #3d3f40;
    }
    .navLink-active {
      color: rgb(255, 255, 255);
      border: 3px solid #8f9393;
      background-color: #3d3f40;
    }
  }
}

</style>
 