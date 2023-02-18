import { Component, useEffect, useRef, useState } from "react";
import { ParagraphSlide } from "./ParagraphSlide";
import { SlideType } from "./SlideTypes";

export type GalleryProps = {
  galleryName: string;
  slides: {
    slideId: string;
    mainContent: string;
    reviewerProfile: {
      picturePath: string;
      name: string;
      title: string;
    };
    slideType: SlideType;
  }[];
};

export default function Gallery(props: GalleryProps) {
  let container = useRef<HTMLDivElement>(null);
  let prevButton = useRef<HTMLAnchorElement>(null);
  let nextButton = useRef<HTMLAnchorElement>(null);
  const [index, setIndex] = useState<number>(0);

  useEffect(() => {
    if (props.slides.length > 1) {
      changeSlide(0);
    } else {
      if (prevButton.current && nextButton.current) {
        prevButton.current.style.display = "none";
        nextButton.current.style.display = "none";
      }
    }
  }, []);

  const [prevSlide, setPrevSlide] = useState<string>("");
  const [nextSlide, setNextSlide] = useState<string>("");

  const changeSlide = (amount: number) => {
    let currentIndex = index + amount;

    if (currentIndex >= props.slides.length) {
      currentIndex = 0;
    } else if (currentIndex < 0) {
      currentIndex = props.slides.length - 1;
    }
    setIndex(currentIndex);

    let prev = index - 1;
    if (prev < 0) {
      prev = props.slides.length - 1;
    }
    let next = index + 1;
    if (next >= props.slides.length) {
      next = 0;
    }
    setPrevSlide(props.slides[prev].slideId);
    setNextSlide(props.slides[next].slideId);
    console.log(
      "prev: " + prevSlide + " current: " + index + " next: " + nextSlide
    );
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
          {props.slides.map((slide) => {
            switch (
              // Used for future proofing in case we want to use gallery again with other type of slides
              slide.slideType
            ) {
              case SlideType.PARAGRAPH: {
                return (
                  <ParagraphSlide
                    id={slide.slideId}
                    key={slide.slideId}
                    paragraph={slide.mainContent}
                    reviewerProfile={slide.reviewerProfile}
                  />
                );
              }
            }
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
