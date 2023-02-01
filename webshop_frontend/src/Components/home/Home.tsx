import React, { Component } from 'react';
import {Partners} from "./Partners";

class Home extends Component {
    state = {
        partners: [
            {name: 'Acme', link: 'https://unsplash.it/', imageSize: [200, 100], imageSource: 'https://unsplash.it/200/100'},
            {name: 'Acme2', link: 'https://unsplash.it/', imageSize: [200, 100], imageSource: 'https://unsplash.it/200/100'},
            {name: 'Acme3', link: 'https://unsplash.it/', imageSize: [200, 100], imageSource: 'https://unsplash.it/200/100'},
        ]
    }
    render() {
        return (
            <React.Fragment>
                <section className="hero">
                    <div className="hero-inner">
                        <h1 className="hero-title">ProFlex</h1>
                        <p className="hero-text">Software for <b>your</b> enterprise</p>
                        <button className="hero-button">Our offers</button>
                    </div>
                </section>
                <hr></hr>
                <section className="container">
                    <h2>Our values</h2>
                    <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Dignissim enim sit amet venenatis urna cursus eget nunc scelerisque. A lacus vestibulum sed arcu non odio euismod lacinia at. Odio ut sem nulla pharetra diam. Ac tortor dignissim convallis aenean et tortor. Massa eget egestas purus viverra accumsan in nisl nisi.</p>
                </section>
                <section className="container">
                    <h2>Our partners</h2>
                    <Partners partners={this.state.partners}/>
                </section>
            </React.Fragment>
        );
    }
}

export default Home;