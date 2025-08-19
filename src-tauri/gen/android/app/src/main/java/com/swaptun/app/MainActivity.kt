package com.swaptun.app
import android.os.Bundle
import android.view.View
import androidx.core.view.ViewCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import android.webkit.WebView
import android.webkit.ValueCallback
import android.util.Log
import android.view.ViewGroup
import android.view.KeyEvent
import android.os.SystemClock
import android.view.MotionEvent
import android.os.Build
import androidx.activity.enableEdgeToEdge


class MainActivity : TauriActivity() {
    private var currentWebView: WebView? = null
    private var isKeyboardVisible: Boolean = false
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
         if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            enableEdgeToEdge()
        } else {
        // Allow content to extend under the system bars
        WindowCompat.setDecorFitsSystemWindows(window, false)
        /*
        // Get the insets controller to manage system UI
        val windowInsetsController = WindowCompat.getInsetsController(window, window.decorView)

        // Hide both status bar and navigation bar
       
        windowInsetsController.apply {
            hide(WindowInsetsCompat.Type.systemBars())
            systemBarsBehavior = WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
        }
        */

        // Make both bars transparent
        window.statusBarColor = android.graphics.Color.TRANSPARENT
        window.navigationBarColor = android.graphics.Color.TRANSPARENT

        // Handle display cutout (notch)
        if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.P) {
            window.attributes.layoutInDisplayCutoutMode = 
                android.view.WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES
        }
        
        ViewCompat.setOnApplyWindowInsetsListener(findViewById(android.R.id.content)) { view: View, insets: WindowInsetsCompat ->
            val bottomInset = insets.getInsets(WindowInsetsCompat.Type.ime()).bottom
            view.setPadding(0, 0, 0, bottomInset)
            insets
        }
        }
        ViewCompat.setOnApplyWindowInsetsListener(findViewById(android.R.id.content)) { view: View, insets: WindowInsetsCompat ->
            val bottomInset = insets.getInsets(WindowInsetsCompat.Type.ime()).bottom
            isKeyboardVisible = bottomInset > 0
            insets
        }

     

    }

    
 

 override fun onWebViewCreate(webView: WebView) {
     currentWebView = webView
     webView.isVerticalScrollBarEnabled = false
    webView.isHorizontalScrollBarEnabled = false
        // Intercept touch events to prevent scrolling
  
    webView.setOverScrollMode(View.OVER_SCROLL_NEVER)
}


override fun dispatchKeyEvent(event: KeyEvent): Boolean {
        if (event.keyCode == KeyEvent.KEYCODE_BACK && event.action == KeyEvent.ACTION_UP) {
                // Vérifie côté JS
                val list = currentWebView!!.copyBackForwardList()
                val canGoBackAndroid = list.currentIndex > 0
                if (canGoBackAndroid){
                    currentWebView!!.evaluateJavascript("history.back();", null)
                    return true
                }
            
        }
        return super.dispatchKeyEvent(event)
    }

}