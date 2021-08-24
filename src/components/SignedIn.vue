<template>
  <div>
    <Slider v-model="value" range />
    <button class="link" style="float: right" @click="logout">Sign out</button>
    <main>
      <form @submit.prevent="saveSecret">
        <fieldset ref="fieldset">
          <label for="greeting" style="display:block; color:var(--gray);margin-bottom:0.5em;">Change greeting</label>
          <div style="display:flex">
            <input v-model="newGreeting" autocomplete="off" id="greeting" style="flex:1" />
            <button id="save" @click="saveSecret" style="border-radius:0 5px 5px 0">Save</button>
          </div>
        </fieldset>
      </form>
    </main>

    <Notification
      v-show="notificationVisible"
      ref="notification"
      :networkId="networkId"
      :msg="'called method: set_greeting'"
      :contractId="contractId"
      :visible="false"
    />
  </div>
</template>

<script>
import { logout } from '../utils'

import Notification from './Notification.vue'

export default {
  name: 'SignedIn',

  beforeMount() {
    if (this.isSignedIn) {
      this.retrieveSavedGreeting()
    }
  },

  components: {
    Notification,
  },

  data: function() {
    return {
      savedGreeting: '',
      newGreeting: '',
      notificationVisible: false,
      value: [20, 50],
    }
  },

  computed: {
    isSignedIn() {
      return window.walletConnection ? window.walletConnection.isSignedIn() : false
    },
    accountId() {
      return window.accountId
    },
    contractId() {
      return window.contract ? window.contract.contractId : null
    },
    networkId() {
      return window.networkId
    },
  },

  methods: {
    retrieveSavedGreeting() {
      //retrieve greeting
      window.contract.get_greeting({ account_id: window.accountId }).then((greetingFromContract) => {
        this.savedGreeting = greetingFromContract
        this.newGreeting = greetingFromContract
      })
    },

    saveSecret: async function() {
      // fired on form submit button used to update the greeting

      // disable the form while the value gets updated on-chain
      this.$refs.fieldset.disabled = true

      try {
        // make an update call to the smart contract
        await window.contract.set_greeting({
          // pass the new greeting
          message: this.newGreeting,
        })
      } catch (e) {
        alert(
          'Something went wrong! ' +
            'Maybe you need to sign out and back in? ' +
            'Check your browser console for more info.'
        )
        throw e //re-throw
      } finally {
        // re-enable the form, whether the call succeeded or failed
        this.$refs.fieldset.disabled = false
      }

      // update savedGreeting with persisted value
      this.savedGreeting = this.newGreeting

      this.notificationVisible = true //show new notification

      // remove Notification again after css animation completes
      // this allows it to be shown again next time the form is submitted
      setTimeout(() => {
        this.notificationVisible = false
      }, 11000)
    },

    logout: logout,
  },
}
</script>
