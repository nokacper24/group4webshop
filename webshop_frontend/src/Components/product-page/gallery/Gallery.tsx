import { useRef, useState } from "react";
import { Testimonial } from "../../../Interfaces";
import { ParagraphSlide } from "./ParagraphSlide";

/**
 * The props of the gallery component.
 */
export type GalleryProps = {
  galleryName: string;
  slides: Testimonial[];
};

/**
 * Renders a gallery with the given slides.
 *
 * @param props the props of the gallery.
 * @returns The Gallery component.
 */
export default function Gallery(props: GalleryProps) {
  let container = useRef<HTMLDivElement>(null);

  const [index, setIndex] = useState<number>(0);
  const [noButtons] = useState<boolean>(props.slides.length <= 1);

  /**
   * Update the index of the current slide.
   *
   * @param amount the amount of slides to change.
   */
  const changeSlide = (amount: number) => {
    let slidesAmount = props.slides.length;

    if (container.current) {
      let newIndex = index + amount;

      // If the new index is out of range, wrap it around
      if (newIndex < 0) {
        newIndex = slidesAmount - 1;
      } else if (newIndex >= slidesAmount) {
        newIndex = 0;
      }

      // Move the slides
      container.current.style.transform = `translateX(-${
        (newIndex * 100) / slidesAmount
      }%)`;

      setIndex(newIndex);
    }
  };

  return (
    <div className={`gallery ${noButtons ? "grid-auto" : ""}`}>
      <button
        style={{ paddingRight: "0.2em" }}
        className={`icon-button slide-button ${
          noButtons ? "display-none" : ""
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
      <div className={`slides-view ${noButtons ? "full-slides-view" : ""}`}>
        <div className="slides-container" ref={container}>
          {props.slides.map((slide) => (
            <ParagraphSlide
              key={slide.testimonial_id}
              paragraph={slide.text}
              imagePath={
                typeof slide.author_pic === "string"
                  ? slide.author_pic
                  : slide.author_pic.name
              }
              name={slide.author}
            />
          ))}
        </div>
      </div>

      <button
        style={{ paddingLeft: "0.2em" }}
        className={`icon-button slide-button ${
          noButtons ? "display-none" : ""
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
