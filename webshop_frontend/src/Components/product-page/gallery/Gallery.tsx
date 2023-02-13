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
      <button className="slide-button" onClick={() => changeSlide(-1)}>&#8249;</button>
      {props.slides.map((prop) => {
        switch (prop.slideType) {
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
      <button className="slide-button" onClick={() => changeSlide(1)}>&#8250;</button>
    </div>
  );
}

function changeSlide(amount: number) {
  //code here to go to next slide, needs to wait on mounting
  //to continue
  index += amount;

}