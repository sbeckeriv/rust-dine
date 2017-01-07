import React, { Component } from 'react';
import './App.css';
import SimpleMapPage from './simple.js';

import MainMapPage from './flux/components/examples/x_main/main_map_page.jsx';

class App extends Component {
  render() {
    return (
	    <SimpleMapPage />
    );
  }
}

export default App;
