pub(crate) fn prompt_for_private_key_and_password() -> anyhow::Result<(String, String)> {
    let private_key = rpassword::prompt_password("Your private key: ")?;

    let password = prompt_for_password()?;
    Ok((private_key, password))
}

pub(crate) fn prompt_for_password() -> anyhow::Result<String> {
    let password = rpassword::prompt_password("Your password: ")?;

    Ok(password)
}

pub(crate) fn prompt_for_new_password() -> anyhow::Result<String> {
    let password = rpassword::prompt_password("Your new password: ")?;

    Ok(password)
}
