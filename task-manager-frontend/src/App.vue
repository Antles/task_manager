<template>
  <header>
    <h1>Task Manager</h1>
    <nav>
      <router-link to="/">Home</router-link>
      <router-link to="/tasks">Tasks</router-link>
      <a href="#" @click.prevent="logout">Logout</a>
    </nav>
  </header>
  <RouterView />
</template>

<script setup>
import { RouterLink, RouterView, useRouter } from 'vue-router';

const router = useRouter();

function logout() {
  localStorage.removeItem('token');
  router.push('/login');
}
</script>

<style scoped>
header{
  line-height: 1.5;
  max-height: 100vh;
}

nav{
  width: 100%;
  font-size: 12xp;
  text-align: center;
  margin-top: 2rem;
}

nav a.router-link-exact-active{
  color: var(--color-text);
}

nav a.router-link-exact-active:hover{
  background-color: transparent;
}

nav a{
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type{
  border: 0;
}
</style>
