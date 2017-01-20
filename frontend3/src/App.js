import React, { Component } from 'react';
import logo from './logo.svg';
import './App.css';
import MapContainer from './basic.js'
import injectTapEventPlugin from 'react-tap-event-plugin';
injectTapEventPlugin();

class App extends Component {
  render() {
    return (
        <div className="App">
          <MapContainer loaded="true" google={window.google}
          />
        </div>
    );
  }
}

export default App;
