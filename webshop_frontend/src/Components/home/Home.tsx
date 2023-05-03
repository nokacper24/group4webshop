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
      <section className="container">
        <h2>Our values</h2>
        <p>
          At Proflex Solutions, we are committed to providing businesses with
          the best products and services to help them succeed. Our core values
          are customer-centricity, innovation, and transparency. We strive to
          provide excellent customer service and build lasting relationships
          with our clients. We are always looking for new ways to innovate and
          improve our products and services.
        </p>
      </section>
      <section className="container">
        <h2>Our partners</h2>
        <Partners />
      </section>
    </>
  );
}
