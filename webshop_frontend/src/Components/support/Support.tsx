import React from "react";
import SupportForm from "./SupportForm";

/**
 * Represents the Support page.
 *
 * @returns The Support component as a JSX element.
 */
export default function Support() {
  return (
    <React.Fragment>
      <section className="container left-aligned">
        <h1>Support</h1>
        <p>
          At Proflex, we understand that our customers are the backbone of our
          business, which is why we prioritize their needs above all else. Our
          dedicated support team is here to ensure that you have the best
          experience possible with our software.
        </p>
        <p>
          Whether you need assistance with installation, troubleshooting, or
          have any other questions, we are here to help. We offer a variety of
          support options, including online resources, e-mail, and phone
          support, to ensure that you get the help you need, when you need it.
        </p>
        <p>
          Additionally, we offer a comprehensive knowledge base that is
          regularly updated with the latest information, making it easy for you
          to find answers to your questions. With Proflex, you can rest assured
          that your satisfaction is our top priority.
        </p>
      </section>
      <SupportForm />
    </React.Fragment>
  );
}
