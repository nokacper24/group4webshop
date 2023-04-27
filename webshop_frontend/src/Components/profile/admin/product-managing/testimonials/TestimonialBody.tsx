import { useEffect, useState } from "react";
import { TestimonialRow, TestimonialRowProps } from "./TestimonialRow";
import { Testimonial } from "../../../../../Interfaces";
type TestimonialBodyProps = {
  testimonials: Testimonial[];
  editTestimonial: (id: number) => void;
  deletTestimonial: (id: number) => void;
};

/**
 * Component that manages and stores the testimonials for a product.
 *
 * @param props the props of the component, must be of type TestimonialBodyProps
 * @returns the React component for the Testimonial body
 */
export function TestimonialBody(props: TestimonialBodyProps) {
  return (
    <div className="accordion-body">
      {props.testimonials.map((testimonial) => {
        return (
          <TestimonialRow
            key={testimonial.testimonial_id}
            testimonial={testimonial}
            editFunction={props.editTestimonial}
            removeFunction={props.deletTestimonial}
          />
        );
      })}
    </div>
  );
}
