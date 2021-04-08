import tkinter

import xmlrpc.client

WIDTH, HEIGHT = 800, 600
SCROLL_STEP = 10


class Browser:
    def __init__(self):
        self.scroll = 0
        self.window = tkinter.Tk()
        self.canvas = tkinter.Canvas(
            self.window,
            width=WIDTH,
            height=HEIGHT
        )
        self.canvas.pack()
        self.window.bind("<Down>", self.scrolldown)
        self.window.bind("<Up>", self.scrollup)
        self.window.bind("<MouseWheel>", self.mouse_wheel)
        self.layout = []

    def load(self, url):
        proxy = xmlrpc.client.ServerProxy("http://127.0.0.1:8080")
        params = {"url": url}
        response = proxy.load_url(params)
        self.layout = response['layout']
        self.render()

    def render(self):
        self.canvas.delete("all")
        for (cursor_x, cursor_y, c) in self.layout:
            self.canvas.create_text(cursor_x, cursor_y - self.scroll, text=c)

    def scrolldown(self, e):
        self.scroll += SCROLL_STEP
        self.render()

    def scrollup(self, e):
        self.scroll -= SCROLL_STEP
        self.render()

    def mouse_wheel(self, e):
        # down
        if e.delta == -120:
            self.scrollup(e)
        else:
            # up
            self.scrolldown(e)


if __name__ == "__main__":
    import sys
    print("starting up")
    Browser().load(sys.argv[1])
    tkinter.mainloop()
