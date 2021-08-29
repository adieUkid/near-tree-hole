<template>
  <div style="background-color: #91d5ff;">
    <Row>
      <Col span="8"></Col>
      <Col span="8">
        <!-- button area -->
        <Row style="margin-top: 40px;margin-bottom: 20px;">
          <Col span="12" style="text-align: left">
            <Button style="height: 45px" type="success" @click="modalVisible = true">Leave a secret</Button>
          </Col>
          <Col span="12" style="text-align: right">
            <div style="display: inline-block;line-height: 32px;margin-right:10px;color: #17233d;">{{ accountId }}</div>
            <Button style="height: 45px" @click="logout">Sign out</Button>
          </Col>
        </Row>
        <!-- card area -->
        <div style="margin-top: 20px;">
          <Card style="margin-bottom: 20px;" v-for="s in secrets.items" :key="s.id">
            <p>{{ s.content }}</p>
            <div style="margin-top:20px" v-if="s.name">
              <div style="display: inline-block;margin-right: 16px;">
                <Avatar shape="square" style="background-color: #87d068" icon="ios-person" />
              </div>
              <div style="display: inline-block;">
                {{ s.name }}
              </div>
            </div>
          </Card>
        </div>
        <div style="margin-bottom: 40px;">
          <Page :total="secrets.count" @on-change="pageChange" />
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
      this.pageChange(1)
    }
  },

  data: function() {
    return {
      secrets: {},
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
    pageChange(n) {
      window.contract.get_secrets({ page_num: n }).then((secrets) => {
        this.secrets = secrets
      })
    },

    logout: logout,

    async ok() {
      if (!this.content) {
        this.$Message.warning('secret content is empty')
        this.btnLoading = false
        return
      }
      try {
        await window.contract.add_secret({ content: this.content, name: this.name })
        this.$Notice.success({ title: 'Succeeded' })
        this.pageChange(1)
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
