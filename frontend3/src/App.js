import React, { Component } from 'react';
import logo from './logo.svg';
import './App.css';
import Container from './basic.js'
import injectTapEventPlugin from 'react-tap-event-plugin';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
injectTapEventPlugin();

class App extends Component {
  render() {
    return (
      <MuiThemeProvider>
        <div className="App">
          <Container loaded="true" google={window.google}
          />
        </div>
      </MuiThemeProvider>
    );
  }
}

export default App;
