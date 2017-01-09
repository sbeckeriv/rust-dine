import React, { Component } from 'react';
import logo from './logo.svg';
import './App.css';
import Container from './basic.js'

class App extends Component {
  render() {
    return (
      <div className="App">
        <Container loaded="true" google={window.google}

        />
      </div>
    );
  }
}

export default App;
