export interface UserGetType {
    readonly userID: number;
    readonly email: string;
    readonly firstName: string;
    readonly lastName: string;
    readonly nickname: string;
    readonly created: string;
    readonly image: string; // Comes back as a URL
    readonly about: string;
    readonly birthday: string;
    readonly private: boolean;
    readonly access: boolean;
}

// For initializing the user in case we want to have some default values
export const User = (props: UserGetType): UserGetType => {
    const override = {
        image: props?.image ? `/api/file/${props.image}` : "/img/no-profile-picture.jpg"
    }

    return {
        ...props,
        ...override
    }
}
