<template>
  <div class="home">
    <div class="app-info-container">
      
      <div class="info-cards">
        <div class="info-card">
          <div class="card-icon">🚀</div>
          <div class="card-content">
            <h3>Tauri版本</h3>
            <p>{{ tauriVersion }}</p>
          </div>
        </div>
        
        <div class="info-card">
          <div class="card-icon">📦</div>
          <div class="card-content">
            <h3>应用名称</h3>
            <p>{{ appName }}</p>
          </div>
        </div>
        
        <div class="info-card">
          <div class="card-icon">🔖</div>
          <div class="card-content">
            <h3>应用版本</h3>
            <p>{{ appVersion }}</p>
          </div>
        </div>
      </div>
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
    display: flex;
    justify-content: center;

    .app-info-container {
      width: 80%;
      border-radius: 20px;
      box-shadow: 0 8px 30px rgba(0, 0, 0, 0.2);
      padding: 30px;
    }
    .info-cards {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
      gap: 20px;
    }

    .info-card {
      display: flex;
      background: rgb(221, 221, 221);
      border-radius: 13px;
      padding: 20px;
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
      transition: all 0.5s ease;
      
      &:hover {
        transform: rotate(-2deg) scale(1.1);
        color: white;
        background: rgb(0, 0, 0);
      }
      
      .card-icon {
        font-size: 2.5rem;
        margin-right: 16px;
        display: flex;
        align-items: center;
      }
      
      .card-content {
        h3 {
          font-size: 1.2rem;
          margin: 0 0 8px 0;
        }
        
        p {
          font-size: 1.1rem;
        }
      }
    }
  }

</style>
 