import { useEffect, useRef, useState } from "react";
import { Testimonial } from "../../../Interfaces";
import { ParagraphSlide } from "./ParagraphSlide";

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

  const [index, setIndex] = useState<number>(0);

  /**
   * Update the index of the current slide.
   *
   * @param amount the amount of slides to change.
   */
  const changeSlide = (amount: number) => {
    let slidesAmount = props.testimonials.length;

    if (container.current) {
      let newIndex = index + amount;

      if (newIndex < 0) {
        newIndex = slidesAmount - 1;
      } else if (newIndex >= slidesAmount) {
        newIndex = 0;
      }

      container.current.style.transform = `translateX(-${
        (newIndex * 100) / slidesAmount
      }%)`;

      setIndex(newIndex);
    }
  };

  return (
    <div className="gallery">
      <button
        style={{ paddingRight: "0.2em" }}
        className={`icon-button slide-button ${
          props.testimonials.length > 1 ? "" : "display-none"
        }`}
        onClick={() => changeSlide(-1)}
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
      </button>
      <div className="slides-view">
        <div
          style={
            props.testimonials.length > 1 ? { gridTemplateColumns: "auto" } : {}
          }
          className="slides-container"
          ref={container}
        >
          {props.testimonials.map((slide) => (
            <ParagraphSlide
              key={slide.testimonial_id}
              paragraph={slide.text}
              imagePath={slide.author_pic}
              name={slide.author}
            />
          ))}
        </div>
      </div>

      <button
        style={{ paddingLeft: "0.2em" }}
        className={`icon-button slide-button ${
          props.testimonials.length > 1 ? "" : "display-none"
        }`}
        onClick={() => changeSlide(1)}
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
      </button>
    </div>
  );
}
