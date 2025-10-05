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

          // Set status bar icons to dark (for light background)
        val controller = WindowCompat.getInsetsController(window, window.decorView)
        controller?.isAppearanceLightStatusBars = true  // Dark icons on light background
        controller?.isAppearanceLightNavigationBars = true  // Dark icons on light background
        // Allow content to extend under the system bars
        WindowCompat.setDecorFitsSystemWindows(window, false)

        // Make both bars transparent
        window.statusBarColor = android.graphics.Color.TRANSPARENT
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            window.isNavigationBarContrastEnforced = false
        }
        window.navigationBarColor = android.graphics.Color.TRANSPARENT

      

        // Handle display cutout (notch)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
            window.attributes.layoutInDisplayCutoutMode =
                android.view.WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES
        }

        ViewCompat.setOnApplyWindowInsetsListener(findViewById(android.R.id.content)) { view: View, insets: WindowInsetsCompat ->
            val bottomInset = insets.getInsets(WindowInsetsCompat.Type.ime()).bottom
            view.setPadding(0, 0, 0, bottomInset)
            insets
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
        if (event.keyCode == KeyEvent.KEYCODE_BACK && event.action == KeyEvent.ACTION_DOWN) {
                // Récupère l'URL actuelle et l'historique
                val currentUrl = currentWebView?.url ?: ""
                val backForwardList = currentWebView?.copyBackForwardList()
                val currentIndex = backForwardList?.currentIndex ?: 0
                val baseUrl = "http://tauri.localhost/home/accueil"
                // Si on peut revenir en arrière dans l'historique
                val canGoBack = currentIndex > 0 && (currentUrl != baseUrl && !currentUrl.endsWith("/home/accueil"))
                
                if (canGoBack) {
                    currentWebView?.evaluateJavascript("history.back();", null)
                    return true // Consomme l'événement pour empêcher le dispatch à la WebView
                }

                // Met l'application en arrière-plan
                moveTaskToBack(true)
                return true

        }
        return super.dispatchKeyEvent(event)
    }

}