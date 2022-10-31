import React, { Component } from 'react';
import './App.css';
import Shelly from './Components/Shelly';

class App extends Component {
  render() {
    return (
      <div className="App">
        <div className="App-header">
          <h2>Shelly Plug S</h2>
        </div>
        <div className="App-intro">
          <Shelly/>
        </div>
      </div>
    );
  }
}

export default App;
