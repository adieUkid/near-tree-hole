<template>
  <div>
    <Row>
      <Col span="8"></Col>
      <Col span="8">
        <!-- button area -->
        <Row style="margin-top: 40px;margin-bottom: 20px;">
          <Col span="4" style="text-align: left">
            <Button type="success" @click="modalVisible = true">Leave a secret</Button>
          </Col>
          <Col span="4" offset="16" style="text-align: right">
            <Button @click="logout">Sign out</Button>
          </Col>
        </Row>
        <Row>
          <!-- card area -->
          <Col span="24">
            <Row style="margin-top: 20px;">
              <Col span="24">
                <Card>
                  <Row>
                    <p>Content of card1231231231 23123123123123</p>
                  </Row>
                  <Row style="margin-top:20px">
                    <Col span="3"><Avatar shape="square" style="background-color: #87d068" icon="ios-person" /></Col>
                    <Col span="20" style="display: flex; justify-content: left; align-items: center;">nobody</Col>
                  </Row>
                </Card>
              </Col>
            </Row>
          </Col>
        </Row>
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
      <Input v-model="secret" type="textarea" :rows="10" placeholder="Enter your secret..." style="width: 100%" />
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
      secret: '',
      name: '',
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
      // <!-- render from secrets -->
      // impl two function

      try {
        await window.contract
          .add_secret({
            secret: this.secret,
            name: this.name,
          })
          .then(function() {
            this.$Notice.success({
              title: 'Succeeded',
            })
            this.retrieveSecrets()
          })
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
        this.secret = ''
        this.name = ''
      }
    },
  },
}
</script>
