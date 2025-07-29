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
  margin-top: 3%;
}
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
  transition: all 0.3s ease;
  
  &:hover {
    transform: translateY(-3px);
    box-shadow: 0 6px 16px rgba(0, 0, 0, 0.35);
  }
  
  .card-icon {
    font-size: 2.5rem;
    margin-right: 16px;
    display: flex;
    align-items: center;
  }
  
  .card-content {
    flex: 1;
    
    h3 {
      font-size: 1.2rem;
      font-weight: 600;
      color: #4a5568;
      margin: 0 0 8px 0;
    }
    
    p {
      font-size: 1.1rem;
      color: #2d3748;
      margin: 0;
      word-break: break-word;
    }
  }
}
</style>
 