import React, { Component } from "react";
import { Link } from "react-router-dom";
import { Partners } from "./Partners";

/**
 * Represents the Home Page of the website.
 *
 * Contains a hero image, an "our values" section, and a "our partners" section.
 */
export default class Home extends Component {
  state = {
    partners: [
      {
        name: "Acme",
        link: "https://logoipsum.com/",
        imageSize: [200, 100],
        imageSource: "https://img.logoipsum.com/289.svg",
      },
      {
        name: "Acme2",
        link: "https://logoipsum.com/",
        imageSize: [200, 100],
        imageSource: "https://img.logoipsum.com/235.svg",
      },
      {
        name: "Acme3",
        link: "https://logoipsum.com/",
        imageSize: [200, 100],
        imageSource: "https://img.logoipsum.com/260.svg",
      },
      {
        name: "Acme4",
        link: "https://logoipsum.com/",
        imageSize: [200, 100],
        imageSource: "https://img.logoipsum.com/254.svg",
      },
      {
        name: "Acme5",
        link: "https://logoipsum.com/",
        imageSize: [200, 100],
        imageSource: "https://img.logoipsum.com/250.svg",
      },
      {
        name: "Acme6",
        link: "https://logoipsum.com/",
        imageSize: [200, 100],
        imageSource: "https://img.logoipsum.com/251.svg",
      },
      {
        name: "Acme7",
        link: "https://logoipsum.com/",
        imageSize: [200, 100],
        imageSource: "https://img.logoipsum.com/253.svg",
      },
    ],
  };

  render() {
    return (
      <React.Fragment>
        <section className="hero">
          <div className="hero-inner">
            <h1 className="hero-title">ProFlex</h1>
            <p className="hero-text">
              Software for <span className="vibrate">your</span> enterprise
            </p>
            <Link className="hero-button" to={"/products"}>
              Offers
            </Link>
          </div>
        </section>
        <hr></hr>
        <section className="container">
          <h2>Our values</h2>
          <p>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
            eiusmod tempor incididunt ut labore et dolore magna aliqua.
            Dignissim enim sit amet venenatis urna cursus eget nunc scelerisque.
            A lacus vestibulum sed arcu non odio euismod lacinia at. Odio ut sem
            nulla pharetra diam. Ac tortor dignissim convallis aenean et tortor.
            Massa eget egestas purus viverra accumsan in nisl nisi.
          </p>
        </section>
        <section className="container">
          <h2>Our partners</h2>
          <Partners partners={this.state.partners} />
        </section>
      </React.Fragment>
    );
  }
}
