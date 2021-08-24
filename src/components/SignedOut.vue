<template>
  <main>
    <div class="logo"><img src="../assets/tree.jpeg" alt="" srcset="" /></div>
    <h1>Welcome to Tree hole!</h1>
    <p>
      A place to keep secrets.
    </p>
    <p>
      Those secrets that are not told in reality, please leave it here!
    </p>
    <p>Nothing will be left except your secret.</p>
    <p style="text-align:center; margin-top:2.5em">
      <button v-on:click="login">Sign in</button>
    </p>
  </main>
</template>

<script>
import { login } from '../utils'

export default {
  name: 'SignedOut',

  methods: {
    login() {
      console.log('calling utils.login')
      login()
    },
  },
}
</script>
