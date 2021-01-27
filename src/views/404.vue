<template>
  <div class="not-found">
    <div id="background"></div>
    <div class="top">
      <h1>404</h1>
      <h3>page not found</h3>
    </div>
    <div class="container">
      <div class="ghost-copy">
        <div class="one"></div>
        <div class="two"></div>
        <div class="three"></div>
        <div class="four"></div>
      </div>
      <div class="ghost">
        <div class="face">
          <div class="eye"></div>
          <div class="eye-right"></div>
          <div class="mouth"></div>
        </div>
      </div>
      <div class="shadow"></div>
    </div>
    <div class="bottom">
      <p>Boo, looks like a ghost stole this page!</p>
      <div class="buttons">
        <vs-button color="primary" type="filled" to="-1" class="btn">
          Back
        </vs-button>
        <vs-button
          color="success"
          type="filled"
          :to="{ name: 'Home' }"
          class="btn"
          >Home</vs-button
        >
      </div>
    </div>

    <footer>
      <p>
        made by
        <a href="https://ranjitghimire.com.np">Amrit Ghimire 'Ranjit'</a> ♡
      </p>
    </footer>
  </div>
</template>

<style scoped lang="scss">
.not-found {
  padding: 32px 0;
  min-height: 68vh;
  justify-content: center;
}
$white: #ededed;
$gray: #bfc0c0;
$dark: #585959;
$light: #d3deea;

@import url("https://fonts.googleapis.com/css?family=Abril+Fatface|Lato");

$big: "Abril Fatface", serif;
$body: "Lato", sans-serif;

body {
  background: $light;
}

.top {
  margin-top: 30px;
}

.container {
  margin: 0 auto;
  position: relative;
  width: 250px;
  height: 250px;
  margin-top: -40px;
}

.ghost {
  width: 50%;
  height: 53%;
  left: 25%;
  top: 10%;
  position: absolute;
  border-radius: 50% 50% 0 0;
  background: $white;
  border: 1px solid $gray;
  border-bottom: none;
  animation: float 2s ease-out infinite;
}

.ghost-copy {
  width: 50%;
  height: 53%;
  left: 25%;
  top: 10%;
  position: absolute;
  border-radius: 50% 50% 0 0;
  background: $white;
  border: 1px solid $gray;
  border-bottom: none;
  animation: float 2s ease-out infinite;
  z-index: 0;
}

.face {
  position: absolute;
  width: 100%;
  height: 60%;
  top: 20%;
}
.eye,
.eye-right {
  position: absolute;
  background: $dark;
  width: 13px;
  height: 13px;
  border-radius: 50%;
  top: 40%;
}

.eye {
  left: 25%;
}
.eye-right {
  right: 25%;
}

.mouth {
  position: absolute;
  top: 50%;
  left: 45%;
  width: 10px;
  height: 10px;
  border: 3px solid;
  border-radius: 50%;
  border-color: transparent $dark $dark transparent;
  transform: rotate(45deg);
}

.one,
.two,
.three,
.four {
  position: absolute;
  background: $white;
  top: 85%;
  width: 25%;
  height: 23%;
  border: 1px solid $gray;
  z-index: 0;
}

.one {
  border-radius: 0 0 100% 30%;
  left: -1px;
}

.two {
  left: 23%;
  border-radius: 0 0 50% 50%;
}

.three {
  left: 50%;
  border-radius: 0 0 50% 50%;
}

.four {
  left: 74.5%;
  border-radius: 0 0 30% 100%;
}

.shadow {
  position: absolute;
  width: 30%;
  height: 7%;
  background: $gray;
  left: 35%;
  top: 80%;
  border-radius: 50%;
  animation: scale 2s infinite;
}

@keyframes scale {
  0% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.1);
  }
  100% {
    transform: scale(1);
  }
}

@keyframes float {
  50% {
    transform: translateY(15px);
  }
}

.bottom {
  margin-top: 10px;
}

/*text styling*/
h1 {
  font-family: $big;
  color: $white;
  text-align: center;
  font-size: 9em;
  margin: 0;
  text-shadow: -1px 0 $gray, 0 1px $gray, 1px 0 $gray, 0 -1px $gray;
}
h3 {
  font-family: $body;
  font-size: 2em;
  text-transform: uppercase;
  text-align: center;
  color: $gray;
  margin-top: -20px;
  font-weight: 900;
}
p {
  text-align: center;
  font-family: $body;
  color: $dark;
  font-size: 0.6em;
  margin-top: -20px;
  text-transform: uppercase;
}

.search {
  text-align: center;
}

.buttons {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: 10px;
}

/*search style*/

.search-bar {
  border: 1px solid $gray;
  padding: 5px;
  height: 20px;
  margin-left: -30px;
  width: 200px;
  outline: none;
  &:focus {
    border: 1px solid $light;
  }
}

.search-btn {
  position: absolute;
  width: 30px;
  height: 32px;
  border: 1px solid $gray;
  background: $gray;
  text-align: center;
  color: $white;
  cursor: pointer;
  font-size: 1em;
  outline: none;
  &:hover {
    background: $white;
    border: 1px solid $white;
    color: $gray;
    transition: all 0.2s ease;
  }
}

.btn {
  padding: 15px 20px;
  margin: 5px;
}

footer {
  position: absolute;
  bottom: 0;
  right: 0;
  text-align: center;
  font-size: 0.8em;
  text-transform: uppercase;
  padding: 10px;
  color: #ea7996;
  letter-spacing: 3px;
  font-family: $body;
  a {
    color: #ffffff;
    text-decoration: none;
    &:hover {
      color: #7d7d7d;
    }
  }
}
</style>
