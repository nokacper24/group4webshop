import { useEffect, useState } from "react";
import { TestimonialRow, TestimonialRowProps } from "./TestimonialRow";
import { Testimonial } from "../../../../../Interfaces";
type TestimonialBodyProps = {
  testimonials: Testimonial[];
  editTestimonial: (id: number) => void;
  deletTestimonial: (id: number) => void;
};

/**
 * Simple component that keeps the rows of the accordion body.
 *
 * @param props the props of the component, must be of AccordionBodyProps type
 * @returns the React component for the Accordion body
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
