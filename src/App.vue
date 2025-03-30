<script setup lang="ts">
import AppHeader from "./components/AppHeader.vue";
import AppSider from "./components/AppSider.vue";
</script>

<template>
  <a-layout class="app">
    <AppHeader></AppHeader>
    <a-layout>
      <AppSider></AppSider>
      <a-layout-content class="content">Content</a-layout-content>
    </a-layout>
  </a-layout>
</template>

<style scoped>
.content {
  text-align: center;
  line-height: 120px;
  /* height: calc(100vh - 50px); */
  color: #fff;
  background-color: #108ee9;
}
</style>
