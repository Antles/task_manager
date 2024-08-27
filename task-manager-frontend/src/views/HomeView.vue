<template>
  <div class="home">
    <div class="hero">
      <h1>Welcome to TaskMaster</h1>
      <p class="tagline">Organize your life, one task at a time.</p>
      <router-link to="/register" class="cta-button">Get Started</router-link>
    </div>

    <div class="features">
      <div class="feature-card">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feature-icon">
          <line x1="8" y1="6" x2="21" y2="6"></line>
          <line x1="8" y1="12" x2="21" y2="12"></line>
          <line x1="8" y1="18" x2="21" y2="18"></line>
          <line x1="3" y1="6" x2="3.01" y2="6"></line>
          <line x1="3" y1="12" x2="3.01" y2="12"></line>
          <line x1="3" y1="18" x2="3.01" y2="18"></line>
        </svg>
        <h2>Manage Tasks</h2>
        <p>Easily create, update, and organize your tasks in one place.</p>
      </div>
      <div class="feature-card">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feature-icon">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
          <polyline points="22 4 12 14.01 9 11.01"></polyline>
        </svg>
        <h2>Track Progress</h2>
        <p>Mark tasks as complete and watch your productivity soar.</p>
      </div>
      <div class="feature-card">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feature-icon">
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
          <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
        </svg>
        <h2>Secure &amp; Private</h2>
        <p>Your tasks are kept secure with user authentication.</p>
      </div>
    </div>
  </div>
</template>

<script setup>
// No additional script needed for this component
</script>

<style scoped>
.home {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

.hero {
  text-align: center;
  background-color: white;
  padding: 3rem;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  margin-bottom: 3rem;
  width: 100%;
  max-width: 600px;
}

h1 {
  font-size: 2.5rem;
  color: #2c3e50;
  margin-bottom: 1rem;
}

.tagline {
  font-size: 1.2rem;
  color: #34495e;
  margin-bottom: 2rem;
}

.cta-button {
  display: inline-block;
  background-color: #39c286;
  color: white;
  padding: 0.8rem 1.5rem;
  font-size: 1rem;
  text-decoration: none;
  border-radius: 5px;
  transition: background-color 0.3s ease;
}

.cta-button:hover {
  background-color: #2eaa76;
}

.features {
  display: flex;
  justify-content: center;
  gap: 2rem;
  width: 100%;
}

.feature-card {
  flex: 1;
  max-width: 350px;
  text-align: center;
  padding: 2rem;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.feature-icon {
  width: 48px;
  height: 48px;
  color: #3498db;
  margin-bottom: 1rem;
}

.feature-card h2 {
  font-size: 1.5rem;
  color: #2c3e50;
  margin-bottom: 1rem;
}

.feature-card p {
  font-size: 1rem;
  color: #34495e;
}

@media (max-width: 1024px) {
  .features {
    flex-direction: column;
    align-items: center;
  }

  .feature-card {
    width: 100%;
    max-width: 500px;
    margin-bottom: 2rem;
  }
}

@media (max-width: 768px) {
  .home {
    padding: 1rem;
  }

  .hero {
    padding: 2rem;
  }

  h1 {
    font-size: 2rem;
  }

  .tagline {
    font-size: 1rem;
  }
}

@media (max-width: 480px) {
  .hero {
    padding: 1.5rem;
  }

  h1 {
    font-size: 1.8rem;
  }

  .cta-button {
    width: 100%;
  }
}
</style>
