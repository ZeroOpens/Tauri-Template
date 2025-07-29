<template>
  <div class="index">
    
    <!-- 欢迎页面 -->
    <transition name="fade" mode="out-in">
      <div class="hello" v-if="show">
        <div class="welcome-card">
          <img src="@/assets/Images/logo.png" alt="Logo" class="welcome-logo">
          <h2 class="welcome-title">欢迎使用 Tauri 模版</h2>
          <p class="welcome-desc">有助于快速开发您的软件</p>
          <button @click="setShow" class="welcome-button">
            <span>进入学习</span>
            <i class="arrow-icon">→</i>
          </button>
        </div>
      </div>
      
      <!-- 内容页面 -->
      <div class="content" v-else>
            <!-- 头部 -->
        <div class="app-header">
          <div class="header-content">
            <img src="/tauri.svg" alt="Tauri Logo" class="logo" />
            <h1>tauri-app</h1>
          </div>
        </div>

        <button @click="setShow" class="back-button">
          <i class="back-icon">←</i>
          <span>返回首页</span>
        </button>

        <!-- 导航 -->
        <div class="index-nav">
          <RouterLink :to="{name: 'home'}" class="navLink" active-class="navLink-active">信息</RouterLink>
          <RouterLink :to="{name: 'pinia'}" class="navLink" active-class="navLink-active">pinia</RouterLink>
          <RouterLink :to="{name: 'request'}" class="navLink" active-class="navLink-active">请求</RouterLink>
          <RouterLink :to="{name: 'function'}" class="navLink" active-class="navLink-active">功能</RouterLink>
        </div>
        
        <!-- 展示区 -->
        <div class="index-content">
          <RouterView v-slot="{ Component }">
            <transition name="slide" mode="out-in">
              <component :is="Component" />
            </transition>
          </RouterView>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
  defineOptions({name: 'Index'})
  import { ref } from 'vue'

  const show = ref(true)

  const setShow = () => {
    show.value = !show.value
  }
</script>

<style scoped lang="scss">
.index {
  // 主页面样式
  .hello {
    display: flex;
    justify-content: center;
    
    .welcome-card {
      border-radius: 20px;
      padding: 50px;
      text-align: center;
      max-width: 80%;
      width: 100%;
      box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
      border: 5px solid rgba(255, 255, 255, 0.8);
      
      
      .welcome-logo {
        display: block;
        width: 200px;
        margin: 0 auto 20px;
      }
      
      .welcome-title {
        font-size: 2rem;
        margin: 0 0 10px;
        color: #24292e;
      }
      
      .welcome-desc {
        font-size: 1.1rem;
        color: #586069;
        margin-bottom: 30px;
      }
      
      .welcome-button {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: 12px 30px;
        border-radius: 50px;
        background: linear-gradient(90deg, #24C8DB, #5CCEDB);
        color: white;
        border: none;
        cursor: pointer;
        font-size: 1.1rem;
        font-weight: 600;
        transition: all 0.3s ease;
        box-shadow: 0 4px 15px rgba(36, 200, 219, 0.3);
        
        .arrow-icon {
          margin-left: 8px;
          font-style: normal;
          transition: transform 0.3s ease;
        }
        
        &:hover {
          background: linear-gradient(90deg, #FFC131, #FFD175);
          box-shadow: 0 4px 15px rgba(255, 193, 49, 0.3);
          transform: translateY(-2px);
          box-shadow: 0 4px 10px #969696;

          
          .arrow-icon {
            transform: translateX(5px);
          }
        }
        
        &:active {
          transform: translateY(1px);
        }
      }
    }
  }

  // 内容页面样式
  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    max-width: 1200px;
    margin: 0 auto;
    width: 100%;
    
    // 头部样式
    .app-header {
      .header-content {
        display: flex;
        align-items: center;
        justify-content: center;
        
        .logo {
          width: 60px;
          height: 60px;
          margin-right: 10px;
          filter: drop-shadow(0 4px 6px rgba(0, 0, 0, 0.1));
          transition: transform 0.3s ease;
          
          &:hover {
            transform: rotate(10deg) scale(1.1);
          }
        }
        
        h1 {
          font-size: 2.5rem;
          font-weight: 700;
          color: #24292e;
          margin: 0;
          background: linear-gradient(90deg, #24C8DB, #FFC131);
          -webkit-background-clip: text;
          background-clip: text;
          -webkit-text-fill-color: transparent;
          text-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
        }
      }
    }

    // 返回按钮样式
    .back-button {
      display: flex;
      align-items: center;
      align-self: flex-start;
      padding: 10px 20px;
      margin-top:  -8%;
      border-radius: 30px;
      background: white;
      color: #24292e;
      border: none;
      cursor: pointer;
      font-size: 1rem;
      box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
      transition: all 0.3s ease;
      
      .back-icon {
        margin-right: 8px;
        transition: transform 0.3s ease;
      }
      
      &:hover {
        background: #f6f8fa;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
        
        .back-icon {
          transform: translateX(-3px);
        }
      }
    }
  }

  // 导航样式
  .index-nav {
    display: flex;
    justify-content: center;
    gap: 50px;
    margin: 7% 0 5px 0;
    flex-wrap: wrap;
    
    .navLink {
      text-decoration: none;
      padding: 12px 30px;
      border-radius: 20px;
      color: #020202;
      background: white;
      border: none;
      font-weight: 500;
      box-shadow: 0 2px 5px rgba(0, 0, 0, 0.5);
      transition: all 0.3s ease;
      
      &:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
      }
      
      &:active {
        transform: translateY(1px);
      }
    }
    
    .navLink-active {
      color: white;
      background: linear-gradient(90deg, #24C8DB, #FFC131);
      box-shadow: 0 4px 15px rgba(36, 200, 219, 0.3);
    }
  }
  
  // 内容区域样式
  .index-content {
    background: rgba(255, 255, 255, 0.9);
    border-radius: 20px;
    padding: 30px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.5);
    flex: 1;
  }
  
  // 过渡动画
  .fade-enter-active,
  .fade-leave-active {
    transition: opacity 0.5s ease;
  }
  
  .fade-enter-from,
  .fade-leave-to {
    opacity: 0;
  }
  
  .slide-enter-active,
  .slide-leave-active {
    transition: all 0.3s ease;
  }
  
  .slide-enter-from {
    opacity: 0;
    transform: translateX(20px);
  }
  
  .slide-leave-to {
    opacity: 0;
    transform: translateX(-20px);
  }
}
</style>
 