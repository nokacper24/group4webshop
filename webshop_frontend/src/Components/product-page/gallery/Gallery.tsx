import { useEffect, useRef, useState } from "react";
import { Testimonial } from "../../../Interfaces";
import { ParagraphSlide } from "./ParagraphSlide";
import { SlideType } from "./SlideTypes";

/**
 * The props of the gallery component.
 */
export type GalleryProps = {
  galleryName: string;
  testimonials: Testimonial[];
};

/**
 * Renders a gallery with the given slides.
 *
 * @param props the props of the gallery, must be of type GalleryProps
 * @returns the gallery HTML element
 */
export default function Gallery(props: GalleryProps) {
  let container = useRef<HTMLDivElement>(null);
  let prevButton = useRef<HTMLAnchorElement>(null);
  let nextButton = useRef<HTMLAnchorElement>(null);
  const [index, setIndex] = useState<number>(0);

  // If there is only one slide, hide the buttons, otherwise the buttons are initialized.
  useEffect(() => {
    if (props.testimonials.length > 1) {
      if (prevButton.current && nextButton.current) {
        prevButton.current.style.display = "flex";
        nextButton.current.style.display = "flex";
      }
      changeSlide(0);
    } else {
      if (prevButton.current && nextButton.current) {
        prevButton.current.style.display = "none";
        nextButton.current.style.display = "none";
      }
    }
  }, [props]);

  const [prevSlide, setPrevSlide] = useState<string>("");
  const [nextSlide, setNextSlide] = useState<string>("");

  /**
   * Updates the index of the current slide and the previous and next slide and
   * changes the href of the buttons to the correct slide.
   *
   * @param amount
   */
  const changeSlide = (amount: number) => {
    let currentIndex = index + amount;

    // If the index is out of bounds, set it to the first or last slide
    if (currentIndex >= props.testimonials.length) {
      currentIndex = 0;
    } else if (currentIndex < 0) {
      currentIndex = props.testimonials.length - 1;
    }
    setIndex(currentIndex);

    let prev = index - 1;
    if (prev < 0) {
      prev = props.testimonials.length - 1;
    }
    let next = index + 1;
    if (next >= props.testimonials.length) {
      next = 0;
    }
    // Update the previous and next slide
    setPrevSlide(props.testimonials[prev].testimonial_id.toString());
    setNextSlide(props.testimonials[next].testimonial_id.toString());
  };

  const reviewerProfile = {
    picturePath: props.testimonials[index].author_pic,
    name: props.testimonials[index].author,
  };

  return (
    <div className="gallery">
      <a
        className="icon-button slide-button"
        href={"#" + prevSlide}
        onClick={() => changeSlide(-1)}
        ref={prevButton}
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
          <title>Previous slide</title>
          <path
            fill="none"
            stroke="currentColor"
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth="48"
            d="M328 112L184 256l144 144"
          />
        </svg>
      </a>
      <div className="slides-view">
        <div className="slides-container" ref={container}>
          {props.testimonials.map((slide) => {
            return (
              <ParagraphSlide
                id={slide.testimonial_id.toString()}
                key={slide.testimonial_id}
                paragraph={slide.text}
                reviewerProfile={reviewerProfile}
              />
            );
          })}
        </div>
      </div>

      <a
        className="icon-button slide-button"
        href={"#" + nextSlide}
        onClick={() => changeSlide(1)}
        ref={nextButton}
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
          <title>Next slide</title>
          <path
            fill="none"
            stroke="currentColor"
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth="48"
            d="M184 112l144 144-144 144"
          />
        </svg>
      </a>
    </div>
  );
}
