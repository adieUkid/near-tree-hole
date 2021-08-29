<template>
  <div>
    <Row>
      <Col style="height: 150px;"></Col>
    </Row>
    <Row type="flex" justify="center">
      <Col span="8" offset="4">
        <div class="logo">
          <img src="../assets/tree.jpeg" alt="" srcset="" />
        </div>
      </Col>
      <Col span="11" offset="1">
        <div>
          <h1>Welcome to Tree hole!</h1>
        </div>
        <div>
          <h3>
            A place to keep secrets.
            <br />
            Nothing will be left except your secret.
          </h3>
        </div>
        <div style="margin-top: 20px;">
          <Button type="success" @click="login" size="large">Sign in</Button>
        </div>
      </Col>
    </Row>
  </div>
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

  data() {
    return {}
  },
}
</script>
