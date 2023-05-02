/**
 * Represents the About page.
 *
 * @returns The About component as a JSX element.
 */
export default function About() {
  return (
    <>
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
        <p>
          We are committed to providing businesses with the best products and
          services to help them succeed. Our core values are
          customer-centricity, innovation, and transparency. We strive to
          provide excellent customer service and build lasting relationships
          with our clients. We are always looking for new ways to innovate and
          improve our products and services. We are committed to being open and
          honest with our clients about our processes and the services we offer.
          With Proflex Solutions, you can trust that you're getting the best
          products and services for your business.
        </p>
      </section>
      <section id="contact-us" className="container left-aligned">
        <h2>Contact us</h2>
        <p>
          <b>Phone number:</b> <a href="tel:+4700000000">+47 000 00 000</a>
        </p>
        <p>
          <b>E-mail:</b>{" "}
          <a href="mailto:group04webshop@gmail.com">group04webshop@gmail.com</a>
        </p>
        <p>
          <b>Address:</b> <br />
          Sandbergvegen 98
          <br />
          6009 Ålesund
          <br />
          Norway
        </p>
      </section>
    </>
  );
}
