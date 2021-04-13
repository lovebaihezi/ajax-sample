const all = [...document.querySelectorAll("div")];

const [username, password, confirm] = all.map((v) => {
  const input = v.querySelector("input");
  const msg = v.querySelector("strong");
  return { input, msg };
});

const status = {
  reg: /^\w{8}$/g,
  confirm: /^$/g,
};

username.input.oninput = async (e) => {
  const res = await fetch("/username", { method: "POST", body: username.input.value });
  const text = await res.text();
  username.msg.innerText = text;
};

password.input.oninput = (e) => {
  if (status.reg.test(password.input.value)) {
    confirm.input.disabled = false;
    status.confirm = new RegExp(`^${password.input.value}$`);
  } else {
    confirm.input.disabled = true;
    password.msg = "your password should be char, length by 8";
  }
};
