import { Component } from "react";
import { ParagraphSlide } from "./ParagraphSlide";
import { SlideType } from "./SlideTypes";

export type SlidesProps = {
  slides: {
    id: string;
    mainContent: string;
    reviewerProfile: {
      picturePath: string;
      name: string;
      title: string;
    };
    slideType: SlideType;
  }[];
};

let index: number;

export default function Gallery(props: SlidesProps) {
  let slides: JSX.Element[];

  return (
    <div className="gallery">
      <button
        className="icon-button slide-button"
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
        <div className="slides-container">
          {props.slides.map((prop) => {
            switch (
              prop.slideType //Used for future prrofing in case we want to use gallery again with other type of slides
            ) {
              case SlideType.PARAGRAPH: {
                return (
                  <ParagraphSlide
                    key={prop.id}
                    paragraph={prop.mainContent}
                    reviewerProfile={prop.reviewerProfile}
                  />
                );
              }
            }
          })}
        </div>
      </div>

      <button
        className="icon-button slide-button"
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

function changeSlide(amount: number) {
  //code here to go to next slide, needs to wait on mounting
  //to continue
  index += amount;
}
