import tkinter

def on_button_click():
    # Define the action to perform when the button is clicked here
    pass

root = tkinter.Tk()

root.title("Made With Rust and Python")  # Fixed the title assignment
root.minsize(width=500, height=500)
root.maxsize(width=600, height=600)

field = tkinter.Entry(root,textvariable="Enter email")
button = tkinter.Button(root, text="Click Me", command=on_button_click)
button.pack()  # Place the button on the window

root.mainloop()
