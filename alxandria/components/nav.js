import Link from "next/link";
import Image from "next/image";

export default function Nav() {
    return (
        <nav>
            <Link href="/">
                <Image
                    id="nav-logo"
                    src="/../public/images/logo.png"
                    height={50}
                    width={50}
                />
            </Link>
        </nav>
    )
}