<script setup>
import DeviceReports from '../componets/DeviceReports.vue'
import Rejestracja from '../componets/Rejestracja.vue'
import UserUI from '../componets/UserUI.vue'
</script>

<template>
  <div class="min-h-screen bg-gray-900 flex flex-col items-center justify-center py-12 relative overflow-hidden">
    <div class="absolute inset-0 bg-black opacity-25"></div>
    <div class="absolute inset-0 grid grid-cols-12 gap-2 transform -skew-y-12">
      <div v-for="i in 100-3" :key="i" class="col-span-1 bg-gray-700 opacity-10"></div>
    </div>
    <div class="max-w-5xl w-full bg-gray-800 p-8 rounded-md custom-shadow relative z-10">
      <h1 class="text-3xl font-bold text-center text-white mb-8">System zgłaszania awarii</h1>
      <div v-if="logging">
        <div v-if="!logged">
          <div class="max-w-md mx-auto bg-gray-700 p-8 rounded-md shadow-2xl mt-8">
            <h2 class="text-2xl font-bold mb-6 text-center text-white">Zaloguj się</h2>
            <form @submit.prevent="login" class="space-y-4">
              <div class="mb-4">
                <label class="block text-gray-300 mb-2">Nazwa użytkownika:</label>
                <input
                  type="text"
                  v-model="username"
                  required
                  class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500"
                />
              </div>
              <div class="mb-4">
                <label class="block text-gray-300 mb-2">Hasło:</label>
                <input
                  type="password"
                  v-model="password"
                  required
                  class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500"
                />
              </div>
              <div class="flex justify-center">
                <button
                  type="submit"
                  class="w-72 bg-orange-600 text-white px-4 py-2 rounded-md hover:bg-orange-700 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:ring-opacity-50"
                >
                  Zaloguj
                </button>
              </div>
            </form>
            <p v-if="errorlog" class="mt-4 text-sm text-red-600">{{ errorlog }}</p>
          </div>
        </div>
        <div v-else>
          <h2 class="text-xl font-semibold mb-4 text-white">Zalogowano jako {{ user.username }} 
            <div class="relative z-20 flex justify-end">
              <button class="bg-orange-600 text-white rounded px-4 py-2 hover:bg-orange-700" @click="logout">Wyloguj się</button>
            </div>
          </h2>
          <UserUI />
        </div>
        <div v-if="!logged" class="mt-4">
          <div class="flex justify-center">
            <button @click="toggleLogin" class="w-72 bg-gray-300 text-gray-700 py-2 px-4 rounded-md shadow hover:bg-gray-400 focus:outline-none">Zarejestruj się</button>
          </div>
        </div>
      </div>
      <div v-else>
        <div v-if="!logged">
          <Rejestracja />
          <div class="flex justify-center">
            <button @click="toggleLogin" class="w-72 bg-gray-300 text-gray-700 py-2 px-4 rounded-md shadow hover:bg-gray-400 focus:outline-none mt-4">Zaloguj się</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.custom-shadow {
  box-shadow: 10px 10px 15px rgba(0, 0, 0, 0.8), -3px -3px 10px rgba(0, 0, 136, 0.2);
}
</style>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';

class User {
    constructor(username, password, gmail, role) {
        this.username = username;
        this.password = password;
        this.gmail = gmail;
        this.role = role;
    }
}

export default {
    data() {
        return {
            username: "",
            password: "",
            logged: false,
            logging: true,
            errorlog: "",
            user: new User("null", "null", "null", "null")
        };
    },
    methods: {
        async login() {
          try {
              const response = await dzien2_backend.zaloguj(this.username, this.password);
              const [isAuthenticated, userOption] = response;

              if (isAuthenticated) {
                  this.logged = true;
                  if (userOption) {
                      this.user = userOption[0];
                      localStorage.setItem('currentUser', JSON.stringify(this.user));
                      localStorage.setItem('isLogged', true);
                  }
              } else {
                  this.errorlog = 'Nieprawidłowa nazwa użytkownika lub hasło';
                  this.user = new User("null", "null", "null", "null");
                  localStorage.removeItem('currentUser');
                  localStorage.setItem('isLogged', false);
              }
          } catch (error) {
              console.error("Wystąpił błąd podczas logowania:", error);
              this.errorlog = error;
          }
        },
        async toggleLogin() {
            this.logging = !this.logging;
        },
        async logout() {
            this.logged = false;
            this.user = new User("null", "null", "null", "null");
            localStorage.removeItem('currentUser');
            localStorage.setItem('isLogged', false);
        }
    },
    created() {
        const storedUser = localStorage.getItem('currentUser');
        const isLogged = JSON.parse(localStorage.getItem('isLogged'));

        if (storedUser && isLogged) {
            this.user = JSON.parse(storedUser);
            this.logged = true;
            this.logging = true;
        }
    }
}
</script>