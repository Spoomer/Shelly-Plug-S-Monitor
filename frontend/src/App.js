import React, { Component } from 'react';
import './App.css';
import EPower from './Components/EPower';

class App extends Component {
  render() {
    return (
      <div className="App">
        <div className="App-header">
          <h2>Shelly Stromverbrauch</h2>
        </div>
        <div className="App-intro">
          <EPower/>
        </div>
      </div>
    );
  }
}

export default App;
