library(shiny)

# Define UI
ui <- fluidPage(
    titlePanel("Simple Shiny App"),
    sidebarLayout(
        sidebarPanel(
            sliderInput("bins",
                        "Number of bins:",
                        min = 1,
                        max = 50,
                        value = 30),
            textInput("greeting",
                      "Enter a greeting:",
                      value = "Hello")
        ),
        mainPanel(
            plotOutput("distPlot"),
            textOutput("greetingText")
        )
    )
)

# Define server logic
server <- function(input, output) {
    output$distPlot <- renderPlot({
        x    <- faithful[, 2]  # Old Faithful Geyser data
        bins <- seq(min(x), max(x), length.out = input$bins + 1)
        hist(x, breaks = bins, col = 'darkgray', border = 'white',
             xlab = 'Waiting time to next eruption (in mins)',
             main = 'Histogram of waiting times')
    })

    output$greetingText <- renderText({
        paste(input$greeting, "Shiny!")
    })
}

# Run the application
shinyApp(ui = ui, server = server)