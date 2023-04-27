export type TestimonialHeaderProps = {
  addTestimonial: () => void;
};

/**
 * The header of the Testimonial section. This component is responsible for rendering the header of the section.
 *
 * @param props the props of the component
 * @returns the React component for the Testimonial section header
 */
export function TestimonialHeader(props: TestimonialHeaderProps) {
  return (
    <>
      <div className="accordion-header-container">
        <button className="accordion-header">
          <p>Testimonials</p>
          <svg
            className="header-icon accordion-button-icon"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 150 320 260"
          >
            <title>Expand</title>
            <desc>Expand to show more info</desc>
            {/* Font Awesome Pro 6.3.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. */}
            <path d="M137.4 374.6c12.5 12.5 32.8 12.5 45.3 0l128-128c9.2-9.2 11.9-22.9 6.9-34.9s-16.6-19.8-29.6-19.8L32 192c-12.9 0-24.6 7.8-29.6 19.8s-2.2 25.7 6.9 34.9l128 128z" />
          </svg>
        </button>
        <button
          className="add-button overlay-button accordion-button-icon"
          onClick={() => props.addTestimonial()}
        >
          <svg
            className="header-icon"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 448 512"
          >
            <title>Add testimonial</title>
            <desc>Add a new testimonial</desc>
            {/* Font Awesome Pro 6.3.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. */}
            <path d="M240 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H176V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H384c17.7 0 32-14.3 32-32s-14.3-32-32-32H240V80z" />
          </svg>
        </button>
      </div>
    </>
  );
}
