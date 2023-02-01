import { Link } from "react-router-dom";

export type ProductProps = {
    product: {
        name: string;
        description: string;
        sourceImage: string;
    
    }
}

export const Product = ({product}: ProductProps) => {
    return (
        <li>
            <Link to={"#"}/>¨
            <img height={200}
                width={300} 
                src={product.sourceImage}>
                alt={"Product image of " + product.name}
            </img>
            <h2>{product.name}</h2>
            <p>{product.description}</p>
        </li>
    )
}