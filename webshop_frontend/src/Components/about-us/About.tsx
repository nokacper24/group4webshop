import React from "react";

export default function About() {
  return (
    <React.Fragment>
      <section className="about-us-banner">
        <div className="dark-overlay wrapper-bottom">
          <div className="container left-aligned">
            <h1>About us</h1>
            <p>
              ProFlex Solutions is a one-stop shop for professional services. We
              offer a comprehensive range of software products and services to
              help businesses streamline their operations and maximize
              efficiency.
            </p>
          </div>
        </div>
      </section>
      <hr></hr>
      <section className="container left-aligned">
        <h2>About our solutions</h2>
        <p>
          Our products include online scheduling software, business process
          automation software, professional tax preparation software,
          professional accounting software, professional legal software, and
          professional financial planning software. Our team of experienced
          professionals offer comprehensive support and consulting services to
          ensure businesses get the most out of their software investments. We
          strive to provide businesses with the tools they need to succeed in
          today's competitive market. With Proflex Solutions, businesses can
          quickly and easily access the resources they need to stay ahead of the
          competition.
        </p>
      </section>
      <section className="container left-aligned">
        <h2>Contact us</h2>
      </section>
    </React.Fragment>
  );
}
