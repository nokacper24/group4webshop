import { Link } from "react-router-dom";

export type ProductProps = {
    props: {
        name: string;
        description: string;
        sourceImage: string;
    }
}

export const Product = ({props: product}: ProductProps) => {
    return (
        <li>
            <Link className="product-card" to={"#"}>
                <img height={200}
                    width={300} 
                    src={product.sourceImage}
                    alt={"Product image of " + product.name}
                    className = "card-image">
                </img>
                <h2>{product.name}</h2>
                <p>{product.description}</p>
            </Link>
        </li>
    )
}