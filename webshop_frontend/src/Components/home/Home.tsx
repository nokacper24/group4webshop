import { Link } from "react-router-dom";
import Partners from "./Partners";

/**
 * Represents the Home page of the website.
 * Contains a hero image, an "our values" section, and an "our partners" section.
 *
 * @returns The Home component as a JSX element.
 */
export default function Home() {
  return (
    <>
      <section className="hero">
        <div className="hero-inner">
          <h1 className="hero-title">ProFlex</h1>
          <p className="hero-text">
            Software for <span className="vibrate">your</span> enterprise
          </p>
          <Link className="hero-button" to={"/products"}>
            Explore
          </Link>
        </div>
      </section>
      <hr></hr>
      <section className="container">
        <h2>Our values</h2>
        <p>
          Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
          eiusmod tempor incididunt ut labore et dolore magna aliqua. Dignissim
          enim sit amet venenatis urna cursus eget nunc scelerisque. A lacus
          vestibulum sed arcu non odio euismod lacinia at. Odio ut sem nulla
          pharetra diam. Ac tortor dignissim convallis aenean et tortor. Massa
          eget egestas purus viverra accumsan in nisl nisi.
        </p>
      </section>
      <section className="container">
        <h2>Our partners</h2>
        <Partners />
      </section>
    </>
  );
}
