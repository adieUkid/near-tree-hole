<template>
  <div>
    <Row>
      <Col span="8"></Col>
      <Col span="8">
        <!-- button area -->
        <Row style="margin-top: 40px;margin-bottom: 20px;">
          <Col span="12" style="text-align: left">
            <Button type="success" @click="modalVisible = true">Leave a secret</Button>
          </Col>
          <Col span="12" style="text-align: right">
            <div style="display: inline-block;line-height: 32px;margin-right:10px;">{{ accountId }}</div>
            <Button @click="logout">Sign out</Button>
          </Col>
        </Row>
        <!-- card area -->
        <div v-for="s in secrets" :key="s.id">
          <Card style="margin-top: 20px;">
            <Row>
              <p>{{ s.content }}</p>
            </Row>
            <Row style="margin-top:20px" v-if="s.name">
              <Col span="3"><Avatar shape="square" style="background-color: #87d068" icon="ios-person" /></Col>
              <Col span="20" style="display: flex; justify-content: left; align-items: center;">{{ s.name }}</Col>
            </Row>
          </Card>
        </div>
      </Col>
      <Col span="8"></Col>
    </Row>
    <Modal
      v-model="modalVisible"
      title="I'll keep your secret."
      @on-ok="ok"
      :loading="true"
      ok-text="That's it."
      cancel-text="Maybe later"
      @on-visible-change="visibleChange"
    >
      <!-- todo: valid -->
      <Input v-model="content" type="textarea" :rows="10" placeholder="Enter your secret..." style="width: 100%" />
      <p style="height: 20px;"></p>
      <Input v-model="name" placeholder="Your name (optional)" style="width: 300px" />
    </Modal>
  </div>
</template>

<script>
import { logout } from '../utils'

export default {
  name: 'SignedIn',

  beforeMount() {
    if (this.isSignedIn) {
      this.retrieveSecrets()
    }
  },

  data: function() {
    return {
      secrets: [],
      content: '',
      name: '',
      accountName: '',
      notificationVisible: false,
      modalVisible: false,
    }
  },

  computed: {
    isSignedIn() {
      return window.walletConnection ? window.walletConnection.isSignedIn() : false
    },
    accountId() {
      return window.accountId
    },
  },

  methods: {
    retrieveSecrets() {
      window.contract.get_secrets().then((secrets) => {
        this.secrets = secrets
      })
    },

    logout: logout,

    async ok() {
      try {
        await window.contract.add_secret({ content: this.content, name: this.name })
        this.$Notice.success({ title: 'Succeeded' })
        this.retrieveSecrets()
      } catch (e) {
        this.$Notice.error({
          title: 'Something went wrong! ',
          desc: 'Maybe you need to sign out and back in? Check your browser console for more info.',
        })
        throw e
      } finally {
        this.modalVisible = false
      }
    },
    visibleChange(show) {
      if (!show) {
        this.content = ''
        this.name = ''
      }
    },
  },
}
</script>
