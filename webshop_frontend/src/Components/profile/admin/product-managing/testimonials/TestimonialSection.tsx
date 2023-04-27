import { useEffect, useState } from "react";
import { ChangeType } from "../Accordion/ChangeTypes";
import { TestimonialRowProps } from "./TestimonialRow";
import { TestimonialHeader } from "./TestimonialHeader";
import { TestimonialBody } from "./TestimonialBody";
import { showTestimonialPopup } from "../Edit-popups/TestimonialPopup";
import { Testimonial } from "../../../../../Interfaces";

/**
 * The props of the AccordionSection component.
 */
export type TestimonialSectionProps = {
  testimonials: Testimonial[];

  sectionId: number;
  productId: string;
};

let latestID = 100;
/**
 * The main component for managing a header and its body.
 *
 * @param props the props of the component, must be of AccordionSectionProps type
 * @returns the React component for the Accordion section
 */
export function TestimonialSection(props: TestimonialSectionProps) {
  const newTestimonial = () => {
    showTestimonialPopup({
      testimonial: undefined,
      informationCallBack: finishCreation,
    });
    function finishCreation(testimonial: Testimonial) {
      testimonial = {
        ...testimonial,
        product_id: props.productId,
        testimonial_id: createID(),
      };
      testimonials.push(testimonial);
      setTestimonials([...testimonials]);
    }
  };

  const createID = (): number => {
    return latestID++;
  };

  /**
   * Deletes a row from the body of the section.
   *
   * @param id the ID of the row to be deleted
   */
  const deleteTestimonial = (id: number) => {
    let newTestimonials = testimonials.filter(
      (testimonial) => testimonial.testimonial_id !== id
    );
    setTestimonials(newTestimonials);
  };

  /**
   * Initializes the process of changing the content of a row.
   *
   * @param id the ID of the row to be edited
   */
  const editTestimonial = (id: number) => {
    console.log("edit: " + id);
    let testimonial = testimonials.find(
      (testimonial) => testimonial.testimonial_id === id
    );
    if (testimonial) {
      showTestimonialPopup({
        testimonial: testimonial,
        informationCallBack: finishEdit,
      });
      function finishEdit(testimonial: Testimonial) {
        testimonials[testimonials.indexOf(testimonial)] = testimonial;
        setTestimonials([...testimonials]);
      }
    }
  };

  const [testimonials, setTestimonials] = useState<Testimonial[]>([
    ...props.testimonials,
  ]);

  return (
    <>
      <TestimonialHeader addTestimonial={newTestimonial}></TestimonialHeader>
      <TestimonialBody
        testimonials={testimonials}
        editTestimonial={editTestimonial}
        deletTestimonial={deleteTestimonial}
      ></TestimonialBody>
    </>
  );
}
