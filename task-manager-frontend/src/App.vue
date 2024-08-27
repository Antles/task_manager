<template>
  <div id="app">
    <header>
      <nav>
        <router-link to="/" class="logo">TaskMaster</router-link>
        <div class="nav-links">
          <router-link to="/">Home</router-link>
          <template v-if="isLoggedIn">
            <router-link to="/tasks">My Tasks</router-link>
            <a href="#" @click.prevent="logout">Logout</a>
          </template>
          <template v-else>
            <router-link to="/login">Login</router-link>
            <router-link to="/register">Register</router-link>
          </template>
        </div>
      </nav>
    </header>

    <main>
      <RouterView />
    </main>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const isLoggedIn = computed(() => !!localStorage.getItem('token'))

function logout() {
  localStorage.removeItem('token')
  localStorage.removeItem('username')
  router.push('/')
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: Arial, sans-serif;
  background-color: #f0f2f5;
  color: #333;
}

#app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

header {
  background-color: #fff;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  max-width: 1200px;
  margin: 0 auto;
  padding: 1rem;
}

.logo {
  font-size: 1.5rem;
  font-weight: bold;
  color: #39c286;
  text-decoration: none;
}

.nav-links a {
  color: #333;
  text-decoration: none;
  margin-left: 1rem;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  transition: background-color 0.3s;
}

.nav-links a:hover {
  background-color: #f0f2f5;
}

.nav-links a.router-link-active {
  background-color: #e6e6e6;
}

main {
  flex-grow: 1;
  display: flex;
  justify-content: center;
  padding: 2rem;
}

@media (max-width: 768px) {
  nav {
    flex-direction: column;
    align-items: flex-start;
  }

  .nav-links {
    display: flex;
    flex-direction: column;
    margin-top: 1rem;
  }

  .nav-links a {
    margin-left: 0;
    margin-bottom: 0.5rem;
  }
}
</style>
