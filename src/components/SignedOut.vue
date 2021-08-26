<template>
  <div>
    <Row>
      <Col style="height: 150px;"></Col>
    </Row>
    <Row type="flex" justify="center">
      <Col span="8" offset="3">
        <div class="logo">
          <img src="../assets/tree.jpeg" alt="" srcset="" />
        </div>
      </Col>
      <Col span="12" offset="1">
        <Row style="height:35%">
          <h1>Welcome to Tree hole!</h1>
        </Row>
        <Row style="height:40%">
          <h3>
            A place to keep secrets.
            <br />
            Nothing will be left except your secret.
          </h3>
        </Row>
        <Row>
          <Button type="success" @click="login" size="large">Sign in</Button>
        </Row>
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
